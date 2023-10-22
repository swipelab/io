module main;

import core{ Result, Error, Future };
import net{ SocketAddr };
import server{ Server, Request, Response };

main: () -> Future = {  
  addr = SocketAddr(addr = [127,0,0,1], port = 9001);
  
  service: (:Request) -> Result<Response> = {
    Ok(Response("Hello"))
  }

  server = Server.bind(addr).serve(service);
  
  if Error(e) = server.await {
    panic(e);
  }
}