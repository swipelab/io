module main;

import core { Result, Error, Future };
import net { SocketAddr };
import http { Server, Request, Response, Ok };

main: () -> Future = {
  addr = SocketAddr{addr: [127,0,0,1], port: 9001};
  
  service: (_:Request) -> Result<Response> = {
    Ok(Response("Hello"))
  }

  server = Server.bind(addr).serve(service);
  
  if Error(e) = server.await {
    panic(e);
  }
}