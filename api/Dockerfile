FROM node:20-alpine

WORKDIR /usr/src/app

COPY index.html ./index.html
COPY api/package*.json ./api/
COPY api ./api
RUN cd api && npm install --omit=dev

EXPOSE 3000
CMD ["node", "api/server.js"]
