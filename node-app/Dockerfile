FROM node:lts-alpine3.22

WORKDIR /app

COPY package.json yarn.lock* ./

RUN npm install

COPY . .

CMD ["sh", "-c", "sleep 5 && npx tsx watch src/server.ts"]

EXPOSE 7070
