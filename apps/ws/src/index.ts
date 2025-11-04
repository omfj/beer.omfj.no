import { Hono } from "hono";
import { cors } from "hono/cors";
import { logger } from "hono/logger";
export { WebSocketRoom } from "./room";

const app = new Hono<{
  Bindings: CloudflareBindings & { WEBSOCKET_ROOM: DurableObjectNamespace };
}>();

app.use(logger());
app.use(
  cors({
    origin: ["beer.omfj.no", "localhost:5173"],
    credentials: true,
    allowMethods: ["GET", "POST"],
    allowHeaders: ["Authorization"],
  })
);

/**
 * Connect to a an event room via WebSocket.
 */
app.get("/event/:id", async (c) => {
  const id = c.req.param("id");

  const event = await c.env.DB.prepare("SELECT * FROM event WHERE id = ?")
    .bind(id)
    .first();

  if (!event) {
    console.error("Event not found");

    return c.json(
      {
        message: "Event does not exist",
      },
      404
    );
  }

  const durableObjectId = c.env.WEBSOCKET_ROOM.idFromName(id);
  const durableObject = c.env.WEBSOCKET_ROOM.get(durableObjectId);

  return durableObject.fetch(c.req.raw);
});

/**
 * Broadcast to all clients connected to the graph room that the event has been
 * updated.
 */
app.post("/event/:id", async (c) => {
  const id = c.req.param("id");
  const apiKey = c.req.header("Authorization");

  if (`Bearer ${c.env.API_KEY}` !== apiKey) {
    console.error("Wrong API KEY");
    return c.json(
      {
        message: "Not allowed",
      },
      403
    );
  }

  const durableObjectId = c.env.WEBSOCKET_ROOM.idFromName(id);
  const durableObject = c.env.WEBSOCKET_ROOM.get(durableObjectId);

  return durableObject.fetch(c.req.raw);
});

export default app;
