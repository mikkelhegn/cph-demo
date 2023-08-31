import { Sqlite, HandleRequest, HttpRequest, HttpResponse} from "@fermyon/spin-sdk"

const encoder = new TextEncoder()

export const handleRequest: HandleRequest = async function(request: HttpRequest): Promise<HttpResponse> {
	const conn = Sqlite.openDefault();
	const result = conn.execute("SELECT * FROM todos;", []);
	const json = JSON.stringify(result.rows);

    return {
      status: 200,
        headers: { "foo": "bar" },
      body: encoder.encode(json).buffer
    }
}
