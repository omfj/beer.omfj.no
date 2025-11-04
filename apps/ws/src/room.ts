export class WebSocketRoom {
  private connections = new Set<WebSocket>();

  async fetch(request: Request): Promise<Response> {
    /**
     * Handle broadcast request.
     */
    if (request.method === "POST") {
      console.log("Broadcasting to clients");

      this.connections.forEach((connection) => {
        if (connection.readyState === WebSocket.OPEN) {
          connection.send("UPDATE");
        }
      });

      return new Response("OK");
    }

    /**
     * Handle WebSocket connection upgrade.
     */
    if (request.headers.get("upgrade") !== "websocket") {
      return new Response("Expected websocket", { status: 400 });
    }

    const { 0: client, 1: server } = new WebSocketPair();
    server.accept();
    this.connections.add(server);

    server.addEventListener("close", () => {
      this.connections.delete(server);
    });

    return new Response(null, {
      status: 101,
      webSocket: client,
    });
  }
}
