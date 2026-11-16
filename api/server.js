const express = require('express');
const path = require('node:path');

const app = express();
const port = Number(process.env.PORT) || 3000;
const homepage = path.resolve(__dirname, '..', 'index.html');

app.get('/', (req, res) => {
  res.sendFile(homepage);
});

app.get('/api/health', (req, res) => {
  res.json({
    status: 'ok',
    service: 'multiverse-perfect-version-api',
    message: 'A perfect version from the multiverse.'
  });
});

app.get('/api/hello', (req, res) => {
  const name = typeof req.query.name === 'string' ? req.query.name.trim().slice(0, 80) : '';
  res.json({
    message: `Hello, ${name || 'traveler'} from the multiverse!`,
    name: name || 'traveler'
  });
});

app.listen(port, () => {
  console.log(`Web app and REST API running at http://localhost:${port}`);
});
