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
    vus: 200,
    duration: '5s',
    summaryTrendStats: ["min", "med", "max", "p(95)", "p(99)", "p(99.9)"],
    thresholds: {
        'http_req_duration': ['p(99)<500'], // 99% of requests must complete below 500ms
        // 'my_trend': ['avg<200'], // Custom threshold for the custom metric
    },
};

export default function () {
    let res = getThunderNote()
    let checkRes = check(res, {
        'status is 200': (r) => (r.status === 200),
    });
}

function getDbNote() {
    return http.get("http://note-app:8080/api/notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
}

function getCachedNote() {
    return http.get("http://note-app:8080/api/cached_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
}

function getThunderNote() {
    return http.get("http://note-app:8080/api/thunder_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
}

export function handleSummary(data) {
    return {
        "scriptReport.html": htmlReport(data),
        stdout: textSummary(data, { indent: " ", enableColors: true })
    };
}

export function teardown(data) {
    // 4. teardown code
    let res = http.get("http://note-app:8080/api/metrics", reqHeader);
    console.log(res.json().data);
}
