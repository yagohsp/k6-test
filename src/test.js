import { uuidv4 } from "https://jslib.k6.io/k6-utils/1.4.0/index.js";
import { check } from "k6";
import http from "k6/http";

export let options = {
  vus: 50,
  duration: "5s",
};

function letterUUID(length = 32) {
  const letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let uuid = "";
  for (let i = 0; i < length; i++) {
    uuid += letters.charAt(Math.floor(Math.random() * letters.length));
  }
  return uuid;
}

export default function() {
  const payload = JSON.stringify({
    nome: `Pessoa`,
    apelido: `Apelido ${letterUUID(16)}`,
    nascimento: "2000-04-09",
    stack: ["C", "C#"],
  });

  const headers = { "Content-Type": "application/json" };
  const res = http.post("http://localhost:9999/programadores", payload, {
    headers,
  });

  check(res, {
    "status é 201 (Created)": (r) => r.status === 201,
  });
}

export function teardown(data) {
  console.log("Test finished!");
  http.get("http://localhost:80");
  // http.del('http://localhost:80');
}
