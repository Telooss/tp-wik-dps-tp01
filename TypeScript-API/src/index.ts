import http from 'http';

const PORT = process.env.PING_LISTEN_PORT || 3000;

const server = http.createServer((req, res) => {
  if (req.url === '/ping' && req.method === 'GET') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify(req.headers));
  } else {
    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end();
  }
});

server.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
