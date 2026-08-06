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
        "以下のRustコードにおいて、静的解析ツールが「{}」の疑い（バグ）を検出しました。\n\
        しかし、これは誤検知(False Positive)の可能性があります。\n\
        提供されたコードの文脈を読み解き、開発者が独自のロジック（マクロ、カスタム関数、エンコーディング処理、条件分岐など）で安全確認を行っているかどうかをステップ・バイ・ステップで推論してください。\n\
        \n\
        【コードの文脈】\n\
        {}\n\
        \n\
        最終的な判定結果として、本当のバグである場合は「True Bug」、安全確認が行われている（誤検知である）場合は「False Positive」という文字列を含めて回答してください。",
        bug_type, code_context
    );

    println!("==================================================");
    println!("[LLM Triage] Requesting evaluation for {}...", bug_type);
    
    let body = json!({
        "model": "qwen2.5-coder:7b",
        "prompt": prompt,
        "stream": false
    });

    let resp = match ureq::post("http://localhost:11434/api/generate")
        .send_json(body) {
            Ok(r) => r,
            Err(e) => {
                println!("[LLM Triage] Failed to connect to LLM: {}", e);
                return true; // Fallback to True Bug if LLM is unavailable
            }
        };

    let result: serde_json::Value = match resp.into_json() {
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
