#!/bin/bash

TURQUOISE='\033[0;36m'
MAGENTA='\033[0;35m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'
pushd () {
    command pushd "$@" > /dev/null
}

popd () {
    command popd "$@" > /dev/null
}

function delete_done_folders {
    emptied_folders=0
    shopt -s globstar
    for folder in **/; do
        if [[ $folder == *"Done"* && -n "$(find "$folder" -mindepth 1 -print -quit 2>/dev/null)" ]]; then
            echo "Deleting files in folder: $folder"
            rm -r "$folder"*
            ((emptied_folders++))
        fi
    done
    shopt -u globstar
    echo "Emptied out $emptied_folders directories."
}

send_to_discord() {
    local webhook_url="DISCORD_WEBHOOK"
    local message="$1"
    local file="$2"
    curl -H "Content-Type: multipart/form-data" -F "file=@$file" "$webhook_url"
}


wait_for_cpu_idle() {
    threshold=3
    count=0  

    while true; do
        cpu_usage=$(top -bn1 | awk '/^%Cpu/ {print $2}')
        if (( $(echo "$cpu_usage < $threshold" | bc -l) )); then
            break
        fi

      
        count=$((count + 1))
        echo -e "Waiting for CPU to idle ($count seconds). Current CPU usage: $cpu_usage% \r"

        sleep 1
    done
}


run_benchmark() {
    local language="$1"
    local iterations="$2"

    echo -e "${BLUE}Running $language benchmark...${NC}"

    pushd "$language" || return
    pushd "shell_scripts" || return

    for script in *.sh; do
        script_name=$(basename "$script" .sh)
        echo -e "${MAGENTA}Running $script_name ${NC}"

        for ((iter = 1; iter <= iterations; iter++)); do
            script_output="$(
                bash "$script" 2>&1
            )"

            echo "Captured Output for $script_name (Iteration $iter):"
            echo "$script_output"

            execution_time=$(echo "$script_output" | grep -oE "Execution Time: [0-9]+\.[0-9]+ seconds" | awk '{print $3}')
            formatted_execution_time=$(printf "%.6f" "$execution_time")

            local_date_time=$(date +"%Y-%m-%d %H:%M:%S")

            echo -e "${GREEN}Execution Time Captured for $script_name ($language - Iteration $iter):${NC} ${RED}$execution_time seconds${NC}"

            pushd "../../benchmarkdata"
            echo "$local_date_time,$language,$script_name,$iter,$formatted_execution_time" >> "$csv_file"
            popd
            sleep 2
        done
    done

    popd || return
    popd || return
    
    echo "Benchmark completed."
}




echo -e "${MAGENTA}Deleting all contents from output folders...${NC}"
delete_done_folders


timestamp=$(date +"%Y%m%d%H%M%S")
csv_file="benchmark_results_${timestamp}.csv"
echo "Timestamp,Language,Script,Iteration,Execution Time (s)" > "benchmarkdata/$csv_file"



echo -e "${TURQUOISE}Choose benchmarking mode (1 for single run, 2 for iterative): ${NC}"
read benchmark_mode


if [[ $benchmark_mode -eq 1 ]]; then
    
    for language_dir in "python" "rust" "cpp" "golang"; do
        run_benchmark "$language_dir" 1
        wait_for_cpu_idle

    done
else
    
    echo -e "${TURQUOISE}Enter the number of iterations for each benchmark: ${NC}"
    read num_iterations

    for language_dir in "python" "rust" "cpp" "golang"; do
        echo "Benchmarking $language_dir..."
        
        run_benchmark "$language_dir" "$num_iterations"

        # popd || continue
    done
fi

pushd "benchmarkdata" 
echo "Generating Chart..."
python3 charts.py "$csv_file"
send_to_discord "CSV Sent" "$csv_file"
send_to_discord "Chart Sent:" "output_plot.jpg"
popd 
