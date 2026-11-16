const express = require('express');
const app = express();
const port = process.env.PORT || 3000;

app.get('/api/health', (req, res) => {
  res.json({
    status: 'ok',
    service: 'multiverse-perfect-version-api',
    message: 'A perfect version from the multiverse.'
  });
});

app.get('/api/hello', (req, res) => {
  res.json({
    message: 'Hello from the multiverse!',
    name: req.query.name || 'traveler'
  });
});

app.listen(port, () => {
  console.log(`REST API running on http://localhost:${port}`);
});
