import os
import sqlite3
import time

from flask import Flask, request, render_template


app = Flask(__name__)

DABDAB_DB = os.getenv("DABDAB_DB")


@app.route("/")
def index():
    return render_template("index.html")


def log_result(worker_id, result):
    with sqlite3.connect(DABDAB_DB) as conn:
        conn.execute("""CREATE TABLE IF NOT EXISTS results(
                worker_id TEXT NOT NULL,
                result TEXT NOT NULL,
                time INTEGER NOT NULL) STRICT""")
        t = int(time.time())
        conn.execute(
            "INSERT INTO results(worker_id,result,time) VALUES (?, ?, ?)",
            (worker_id, result, t),
        )


def read_results(from_time=None):
    with sqlite3.connect(DABDAB_DB) as conn:
        if from_time is not None:
            cur = conn.execute(
                "SELECT worker_id,result,time FROM results WHERE time >= ?",
                from_time,
            )
        else:
            cur = conn.execute("SELECT worker_id,result,time FROM results")
        return [
            {"worker_id": r[0], "result": r[1], "time": r[2]} for r in cur.fetchall()
        ]


@app.post("/results")
def post_results():
    worker_id = request.form.get("worker_id")
    result = request.form.get("result")
    if worker_id is None:
        return "Missing 'worker_id' parameter", 400
    elif result is None:
        return "Missing 'result' parameter", 400
    log_result(worker_id, result)
    return "Result submitted"


@app.get("/results")
def get_results():
    return read_results()
