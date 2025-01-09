import http from 'k6/http';
import { check } from 'k6';
import { Counter } from 'k6/metrics';
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

export const requests = new Counter('http_reqs');

const reqHeader = {
    headers: { 'Content-Type': 'application/json' },
}

export let options = {
    stages: [
        { duration: '5s', target: 100 }, // Ramp-up to 100 VUs
        { duration: '5s', target: 200 },  // Ramp-up to 200 VUs
        { duration: '5s', target: 0 },  // Ramp-down to 0 VUs
    ],
    summaryTrendStats: ["min", "med", "max", "p(95)", "p(99)", "p(99.9)"],
    thresholds: {
        'http_req_duration': ['p(99)<500'], // 95% of requests must complete below 500ms
        // 'my_trend': ['avg<200'], // Custom threshold for the custom metric
    },
};

export default function () {
    // let res = http.get("http://note-app:8080/api/notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
    // let res = http.get("http://note-app:8080/api/cached_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
    let res = http.get("http://note-app:8080/api/thunder_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
    let checkRes = check(res, {
        'status is 200': (r) => r.status === 200,
    });
    // myTrend.add(res.timings.duration);
}

export function handleSummary(data) {
    return {
        "scriptReport.html": htmlReport(data),
        stdout: textSummary(data, { indent: " ", enableColors: true })
    };
}