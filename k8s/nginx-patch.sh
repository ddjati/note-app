#!/bin/bash
kubectl patch deployment -n ingress-nginx ingress-nginx-controller \
    --patch="nginx-patch.json"
