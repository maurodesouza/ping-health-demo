FROM node:20-bullseye

WORKDIR /app

RUN npm install -g prisma

COPY prisma ./prisma
COPY package.json ./

RUN npm install

RUN chown -R node:node ./node_modules
RUN chown -R node:node ./prisma

USER node

CMD ["sh", "-c", "sleep 5 && prisma migrate deploy && npx prisma studio --port 5555"]

EXPOSE 5555
