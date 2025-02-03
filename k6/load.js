import http from 'k6/http';
import { check } from 'k6';
import { Counter, Rate, Trend } from 'k6/metrics';
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

export const requests = new Counter('http_reqs');

// const scenario = ['Direct DB test summary', getDbNote];
// const scenario = ['Caching test summary', getCachedNote];
const scenario = ['Thundering Herd Prevention test summary', getThunderNote];

// Define custom metrics
const isFromDbCounter = new Counter('db_hit_counter');
const isFromDbRate = new Rate('db_hit_rate');
const dbDurationTrend = new Trend('db_duration_micros_trend');

const reqHeader = {
    headers: { 'Content-Type': 'application/json' },
}

export let options = {
    vus: 300, // 300 users simultaneously
    duration: '5s', // load test for 5s
    summaryTrendStats: ["min", "p(20)", "med", "max", "p(95)", "p(99)", "p(99.9)"],
    thresholds: {
        'http_req_duration': ['p(99)<500'], // 99% of requests must below 500ms
    },
};

export default function () {
    let res = scenario[1]()
    check(res, {
        'status is 200': (r) => (r.status === 200)
    });

    // Parse the JSON response
    const jsonRes = JSON.parse(res.body);

    check(jsonRes, {
        'contains correct key value': (obj) => obj.data.note.title === 'danang title'
    });

    if (jsonRes.hasOwnProperty('metrics') && jsonRes.metrics.hasOwnProperty('is_from_db')) {
        isFromDbCounter.add(jsonRes.metrics.is_from_db);
        isFromDbRate.add(jsonRes.metrics.is_from_db);
    } else {
        isFromDbRate.add(false);
    }

    if (jsonRes.hasOwnProperty('metrics') && jsonRes.metrics.hasOwnProperty('db_duration')) {
        dbDurationTrend.add(jsonRes.metrics.db_duration);
    }
}

function getDbNote() {
    // return http.get("http://note-app:8080/api/notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
    return http.get("http://localhost:8080/api/notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
}

function getCachedNote() {
    // return http.get("http://note-app:8080/api/cached_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
    return http.get("http://localhost:8080/api/cached_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
}

function getThunderNote() {
    // return http.get("http://note-app:8080/api/thunder_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
    return http.get("http://localhost:8080/api/thunder_notes/f1cd96ca-0515-49de-be6d-3e238748668e", reqHeader);
}

export function handleSummary(data) {
    return {
        stdout: scenario[0] + `\n\n${textSummary(data, { indent: ' ', enableColors: true })}`,
    };

}
