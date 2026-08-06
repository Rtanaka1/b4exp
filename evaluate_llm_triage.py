import os
import glob
import sys

def analyze_reports(base_path):
    report_files = glob.glob(os.path.join(base_path, "*", "report.txt"))
    
    if not report_files:
        print(f"No report.txt files found in {base_path}/*/")
        sys.exit(1)
        
    total_analyzed = 0
    total_static_hits = 0
    filtered_by_llm = 0
    retained_bugs = 0
    
    per_package = {}
    
    for report_file in report_files:
        pkg_name = os.path.basename(os.path.dirname(report_file))
        total_analyzed += 1
        
        static_hits = 0
        filtered = 0
        retained = 0
        
        with open(report_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            
            # TypePulse LLM Triage blocks are enclosed by "=================================================="
            parts = content.split("==================================================")
            
            for part in parts:
                if "[LLM Triage] Requesting evaluation" in part:
                    static_hits += 1
                    # Check if the LLM flagged it as a false positive
                    if "False Positive" in part or "誤検知" in part:
                        filtered += 1
                    else:
                        retained += 1
                        
        total_static_hits += static_hits
        filtered_by_llm += filtered
        retained_bugs += retained
        
        per_package[pkg_name] = {
            'static_hits': static_hits,
            'filtered': filtered,
            'retained': retained
        }
            
    print("==================================================")
    print("           LLM Triage Evaluation Results          ")
    print("==================================================")
    print(f"Total Packages Analyzed: {total_analyzed}")
    print(f"Total Bugs Detected by Static Analysis (Pre-LLM): {total_static_hits}")
    
    if total_static_hits > 0:
        filtered_pct = (filtered_by_llm / total_static_hits) * 100
        retained_pct = (retained_bugs / total_static_hits) * 100
        print(f"Bugs Filtered out as False Positives by LLM: {filtered_by_llm} ({filtered_pct:.1f}%)")
        print(f"Bugs Retained as True Bugs by LLM:           {retained_bugs} ({retained_pct:.1f}%)")
    else:
        print("Bugs Filtered out as False Positives by LLM: 0 (0.0%)")
        print("Bugs Retained as True Bugs by LLM:           0 (0.0%)")
        
    print("\n--- Breakdown by Package (Packages with hits only) ---")
    found_any = False
    for pkg, stats in sorted(per_package.items(), key=lambda x: x[1]['static_hits'], reverse=True):
        if stats['static_hits'] > 0:
            found_any = True
            print(f"{pkg.ljust(30)}: {stats['static_hits']:3d} static -> {stats['retained']:3d} retained ({stats['filtered']:3d} filtered)")
            
    if not found_any:
        print("No packages had any static analysis hits.")

if __name__ == "__main__":
    crates_base_path = "/home/crates/sources/"
    if len(sys.argv) > 1:
        crates_base_path = sys.argv[1]
    
    analyze_reports(crates_base_path)
