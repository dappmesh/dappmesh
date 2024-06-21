#!/bin/bash

COLOR_GREEN="\033[0;92m"
COLOR_RESET="\033[0m"

service="trust-manager"
namespace="cert-manager"
jsonpath='{.subsets[*].addresses[*].ip}'

interval=$((5 * 1000))  # 5 seconds
warmup=$((10 * 1000))   # 10 seconds
timeout=$((60 * 1000))  # 60 seconds
now=$(date +%s%3N)      # Current time in milliseconds
end_time=$((now + timeout))

while [ "$now" -lt $end_time ]; do
    output=$(kubectl get endpoints "$service" -n "$namespace" -o "jsonpath=$jsonpath")

    if [ -z "$output" ] || [ "$output" == "''" ]; then
        idle=1
    else
        idle=0
    fi

    echo -e "${COLOR_GREEN}deploy:trust-manager-wait${COLOR_RESET} Endpoint [ip=$output, idle=$idle]"
    if [ $idle -eq 0 ]; then
        echo -e "${COLOR_GREEN}deploy:trust-manager-wait${COLOR_RESET} Endpoint for service $service is active"
        sleep $((warmup / 1000))
        end_time=$((now - 1000))
    else
        echo -e "${COLOR_GREEN}deploy:trust-manager-wait${COLOR_RESET} Waiting for endpoint of service $service to be active..."
        sleep $((interval / 1000))
        now=$(date +%s%3N)
    fi
done
