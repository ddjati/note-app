#!/bin/bash
eval $(minikube docker-env)
docker build -t danangdjati/note-app:1.0.0 .
