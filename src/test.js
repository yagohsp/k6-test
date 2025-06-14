import http from 'k6/http';
import { check } from 'k6';
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';

export let options = {
  vus: 50,
  duration: '5s',
};

export default function () {
  const payload = JSON.stringify({
    nome: `Pessoa ${uuidv4()}`,
    email: `teste-${Math.random().toString(36).substring(7)}@email.com`,
    idade: Math.floor(Math.random() * 60) + 18
  });

  const headers = { 'Content-Type': 'application/json' };
  const res = http.post('http://localhost:80', payload, { headers });

  check(res, {
    'status é 201 (Created)': (r) => r.status === 201,
  });
}


export function teardown(data) {
  console.log('Test finished!');
  http.get('http://localhost:80');
  http.del('http://localhost:80');
}
