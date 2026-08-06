'''
This file aims to focus on the True Positives detected by TypePulse with interprocedural mode
Reports will be saved to report.txt and zreport.txt in paths of packages
Compare the results in report3k.csv
Note:
spl-token-swap has broken dependencies issue now
: https://github.com/solana-labs/solana-program-library/issues/5243
cortex-m has been patched in the dependencies
: https://github.com/rust-embedded/cortex-m/issues/485
which have affected our detection results currently
The results in report3k.csv is cleaned after removing duplicated implementations
'''
import os, csv, re, subprocess

def extract_bugs_from_report(report_path):
    bugs = []

    print("Package: ", report_path.split("/")[-2])

    try:
        with open(report_path, "r", encoding = "utf-8") as report_file:
            for line in report_file:
                if "Error (BrokenLayout:): Potential broken layout issue in" in line:
                    print("Bug1: ", line)
                if "Error (UninitExposure:): Potential uninit exposure issue in" in line:
                    print("Bug2: ", line)
                if "Error (BrokenBitPatterns:): Potential broken bit patterns issue" in line:
                    print("Bug3: ", line)
    except FileNotFoundError:
        print(f"File not found: {report_path}")
    except Exception as e:
        print(f"Error reading file {report_path}: {e}")

def collect_bugs_from_dirs(directories):
    for directory in directories:
        report_path = os.path.join(directory, "report.txt")
        zreport_path = os.path.join(directory, "zreport.txt")
        if os.path.exists(report_path):
            print("with interprocedural analysis")
            extract_bugs_from_report(report_path)
            print("disable interprocedural analysis")
            extract_bugs_from_report(zreport_path)


def run_typepulse(directory):
    if not os.path.exists(directory):
        print("Not downloaded successfully: ", directory)
        return
    print("Run typepulse in ", directory)
    os.chdir(directory)
    # subprocess.call("rustup override set nightly-2023-06-02-x86_64-unknown-linux-gnu", shell=True)
    subprocess.call("cargo typepulse -j 32 > report.txt 2>&1", shell=True)
    subprocess.call("cargo typepulse -j 32 -- -Zdisable-inter > zreport.txt 2>&1", shell=True)


def main():
    reported_directories = [
        "/home/crates/sources/candle-core-0.4.1",
        "/home/crates/sources/py-spy-0.3.14",
        "/home/crates/sources/fyrox-core-0.27.0",
        "/home/crates/sources/gfx-backend-gl-0.9.0",
        "/home/crates/sources/webrender-0.61.0",
        "/home/crates/sources/silicon-0.5.2",
        "/home/crates/sources/scryer-prolog-0.9.4",
        "/home/crates/sources/libafl-0.10.1",
        "/home/crates/sources/mesalink-1.1.0",
        "/home/crates/sources/fontdue-0.8.0",
        "/home/crates/sources/pprof-0.13.0",
        "/home/crates/sources/rendy-core-0.5.1", 
        "/home/crates/sources/rendy-util-0.4.1",
        "/home/crates/sources/sciter-rs-0.5.58",
        "/home/crates/sources/cortex-m-0.7.7",
        "/home/crates/sources/rosrust-0.9.11",
        "/home/crates/sources/rafx-base-0.0.15",
        "/home/crates/sources/xous-0.9.50",
        "/home/crates/sources/spl-token-swap-3.0.0"
    ]
    
    for rd in reported_directories:
        run_typepulse(rd)

    collect_bugs_from_dirs(reported_directories)


if __name__ == "__main__":
    main()

