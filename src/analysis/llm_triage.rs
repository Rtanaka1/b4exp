use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use serde_json::json;

pub fn get_snippet(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    if let Some(local_def_id) = def_id.as_local() {
        let hir_id = tcx.hir().local_def_id_to_hir_id(local_def_id);
        let span = tcx.hir().span_with_body(hir_id);
        if let Ok(snippet) = tcx.sess.source_map().span_to_snippet(span) {
            return snippet;
        }
    }
    String::from("<source not available>")
}

pub fn evaluate_bug(
    tcx: TyCtxt<'_>,
    bug_type: &str,
    target_func_id: DefId,
    constructor_func_id: Option<DefId>,
) -> bool {
    let target_code = get_snippet(tcx, target_func_id);
    let mut code_context = format!("Target Function:\n```rust\n{}\n```\n", target_code);

    if let Some(ctor_id) = constructor_func_id {
        let ctor_code = get_snippet(tcx, ctor_id);
        code_context.push_str(&format!("\nConstructor / Caller Function:\n```rust\n{}\n```\n", ctor_code));
    }

    let prompt = format!(
        "あなたはRustの専門的なセキュリティ研究者です。\n\
        以下のRustコードに対して、静的解析ツールが「{}」の脆弱性の疑いを検出しました。\n\
        この静的解析ツールは、関数をまたぐ事前確認や、if文による早期リターンなどの複雑な制御フローを追跡できないため、多くの誤検知（False Positive）を出してしまいます。\n\
        あなたの任務は、コード全体を読み解き、この警告が本物の脆弱性（True Bug）か、誤検知（False Positive）かを判定することです。\n\
        \n\
        【誤検知（False Positive）と判定する条件】\n\
        以下のいずれかの「安全保障のロジック」がコード上で明確に確認できる場合は、静的解析の誤検知であるため「False Positive」と判定してください。\n\
        1. 早期リターンによる事前チェック: unsafe操作が行われる前に、if文などでサイズや境界（len, size_of等）を検証し、条件を満たさない場合は return Err(...) などで早期に処理を中断している。\n\
        2. バリデーション関数の実行: unsafe操作の前に validate_...() や is_valid() などの検証関数が呼ばれ、不正なデータが伝播しないよう防いでいる（?演算子によるエラー伝播も含む）。\n\
        3. 型サイズに応じた正確な計算: match size_of::<T>() などで型のサイズに基づき、スライスの長さやバッファオフセットが正確かつ安全に計算されている。\n\
        4. 明示的なアサーション: assert! 等により安全性が事前確認されている。\n\
        \n\
        【True Bugと判定する条件】\n\
        上記の安全保障が一切見当たらず、ポインタの不正アクセスやオーバーランが実際に起こり得る場合は、見逃しを防ぐため必ず「True Bug」と判定してください。単なる推測で安全と見なしてはいけません。\n\
        \n\
        【コードの文脈】\n\
        {}\n\
        \n\
        必ず回答の最初の1行目に【結論: True Bug】または【結論: False Positive】と明記してください。その後の行で「どの行でどのようなチェックが行われているか」を具体的に挙げながら、簡潔に理由を説明してください。",
        bug_type, code_context
    );

    println!("==================================================");
    println!("[LLM Triage] Requesting evaluation for {}...", bug_type);
    println!("[LLM Prompt]\n{}\n--------------------------------------------------", prompt);
    
    let body = json!({
        "model": "qwen2.5-coder:7b",
        "prompt": prompt,
        "stream": false,
        "options": {
            "num_predict": 500
        }
    });

    let body_str = body.to_string();

    let output = std::process::Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("http://localhost:11434/api/generate")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(&body_str)
        .output();

    let resp_str = match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).into_owned()
        }
        Ok(out) => {
            println!("[LLM Triage] curl returned error status: {}", out.status);
            return true;
        }
        Err(e) => {
            println!("[LLM Triage] Failed to execute curl: {}", e);
            return true; // Fallback to True Bug if LLM is unavailable
        }
    };

    let result: serde_json::Value = match serde_json::from_str(&resp_str) {
        Ok(v) => v,
        Err(_) => return true,
    };

    let response_text = result["response"].as_str().unwrap_or("");
    println!("[LLM Output]\n{}\n", response_text);
    println!("==================================================");

    if response_text.contains("False Positive") || response_text.contains("誤検知") {
        false // Not a bug
    } else {
        true // Is a bug
    }
}
