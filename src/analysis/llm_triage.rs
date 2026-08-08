use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use serde_json::json;

pub fn get_snippet(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    if let Some(local_def_id) = def_id.as_local() {
        let hir_id = tcx.hir().local_def_id_to_hir_id(local_def_id);
        let span = tcx.hir().span(hir_id);
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
        あなたの任務は、この警告が本物の脆弱性（True Bug）であるか、それとも完全に安全性が確保されている誤検知（False Positive）であるかを厳格に判定することです。\n\
        \n\
        【厳格な判定基準】\n\
        以下のいずれかの「決定的な安全保障」がコード上で明確に確認できない限り、必ず「True Bug」と判定してください。単なるif文の存在や推測だけで安全と見なしてはいけません。\n\
        1. unsafe操作の直前で、メモリサイズ・境界・アライメントのチェックが確実に行われている。\n\
        2. バッファオーバーランや未初期化メモリの読み取りを完全に防ぐアサーションが存在する。\n\
        3. 不正なキャストを防ぐための厳密な型の検証ロジックがある。\n\
        \n\
        【コードの文脈】\n\
        {}\n\
        \n\
        推論は簡潔に3行以内でまとめ、最終的な結論として、脆弱性の疑いが残る場合は「True Bug」、確実に安全だと言い切れる場合のみ「False Positive」という文字列を必ず含めて回答してください。",
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
            "num_predict": 300
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
