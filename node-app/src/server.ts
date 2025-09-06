import fastify from "fastify"
import { knex as setupKnex } from "knex"
import { v7 as uuid } from "uuid"

const knex = setupKnex({
    client: "pg",
    connection: process.env.DATABASE_URL,
    searchPath: ['knex', 'public']
})

const app = fastify();
const PORT = process.env.PORT || 7070


app.post("/ping", async (_, reply) => {
    const records = await knex.select("*").from("pings").where("service", "node");

    if (!records.length) {
        const [record] = await knex
            .insert({
                id: uuid(),
                amount: 1,
                service: "node"
            })
            .into("pings").returning("*");


        return reply.status(201).send(record)
    }

    const [record] = await knex
        .increment("amount", 1)
        .update("updated_at", new Date())
        .where("service", "node")
        .into("pings")
        .returning("*")

    return record;
})

app.get("/health", async (_, reply) => {
    try {
        await knex.raw('select 1')

        reply.status(200).send()
    } catch (err) {
        reply.status(503).send()
    }
})

app.listen({
    host: "0.0.0.0",
    port: Number(PORT)
})
    .then(() => console.log(`🚀 server is running at port: ${PORT}`))
    .catch((err) => console.error(`🚨 error on starting server: ${err}`))
