# CPH-Demo

This repo is a demo I built for a few talks in Copenhagen in August 2023.

The code in here is the end result, from having run the following steps:

## Build a Rust component to get and set data from the KV Store

1. Create a new Spin app using the http-rust template
    `spin new http-rust rust`

2. Add the SpinSDK router to the component:
    
    1. Replace the code to the main handler in src/lib.rs, with the following
    ```
    		let router = http_router! {
				GET "/rust/:id" => get_stuff,        
				POST "/rust/:id" => set_stuff,
        _ "/*" => |_req, _params| {
            Ok(http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(None)
                .unwrap())
        }
    };

    router.handle(req)
    ```

    2. Add the following import to the `spin::sdk` use statement in src/lib.rs:
    ```
    http_router
    ```
3. Add the two functions to get and set data in the KV store:

    1. Add the following code to src/lib.rs
    ```
    fn set_stuff(req: Request, params: Params) -> Result<Response> {
        let store = Store::open_default()?;

        let id = params.get("id").unwrap();

        match store.exists(id) {
            Ok(true) => println!("Updating key {} in the KV Store", id),
            Ok(false) => println!("Storing key {} in the KV Store", id),
            Err(error) => println!("Help!!! {}", error),
        };

        store.set(id, req.body().as_deref().unwrap_or(&[]))?;

        return Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)?);
    }

    fn get_stuff(_req: Request, params: Params) -> Result<Response> {
        let store = Store::open_default()?;

        let id = params.get("id").unwrap();

        let (body, code) = match store.get(id) {
            Ok(value) => (value.to_vec(), http::StatusCode::OK),
            Err(KeyValueError::NoSuchKey) => (
                "Key not found".as_bytes().to_vec(),
                http::StatusCode::NOT_FOUND,
            ),
            Err(error) => (
                format!("{}", error).as_bytes().to_vec(),
                http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        return Ok(http::Response::builder()
            .status(code)
            .body(Some(body.into()))?);
    }
    ```

    2. Replace the `spin::sdk` use statement to the following:
    ```
    use spin_sdk::{
        http::{Params, Request, Response},
        http_component, http_router,
        key_value::{Error as KeyValueError, Store},
    };
    ```

    3. Add `key_valuestores = ["default"]` to the rust component in `spin.toml`

4. Add the KV-Explorer to the app

    `spin add kv-explorer`

## Deploy to cloud

1. `spin deploy`

## Add a TS Component to get data from a sqlite database

1. Add the new component `spin add http-ts ts`

2. Replace the `index.ts` file content with the following

    ```
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
    ```
3. Add `sqlite_databases = ["default"]` to `spin.toml` for the ts component

4. add a new SQL migration file named 'db.sql' with the following content:

    ```
    CREATE TABLE IF NOT EXISTS todos (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        description TEXT NOT NULL,
        due_date DATE
    );

    INSERT INTO todos (description) VALUES ('Complete the demo')";
    ```

5. Run the migration on next `spin up`
    `spin up --sqlite @db.sql`

## Add a front-end

1. Add the static fileserver `spin add static-fileserver`

2. Create a directory name `assets` with a file name `index.html`

3. Add the following content to the file:
    ```
    <!DOCTYPE html>
    <html>
    <head>
    <title>Backend Service Demo</title>
    <style>
        body {
        background-color: #0D203F;
        font-family: Arial, sans-serif;
        margin: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        }
        .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 400px;
        padding: 20px;
        background-color: #ffffff;
        border-radius: 10px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
        }
        .button {
        margin-top: 10px;
        padding: 10px 20px;
        background-color: #34E8BD;
        color: #ffffff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        width: 100%;
        font-size: 16px;
        }
        .result {
        margin-top: 10px;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        background-color: #f9f9f9;
        font-size: 14px;
        width: 100%;
        }
        .input-field {
        margin-top: 10px;
        padding: 5px;
        border: 1px solid #ccc;
        border-radius: 5px;
        width: 100%;
        }
    </style>
    </head>
    <body>
    <div class="container">
        <h2 style="text-align: center;">Backend Service Demo</h2>
        <input class="input-field" type="text" placeholder="Input for Service 1" id="input1">
        <button class="button" id="button1">Call Service 1</button>
        <div class="result" id="result1"></div>
        <button class="button" id="button2">Call Service 2</button>
        <div class="result" id="result2"></div>
    </div>

    <script>
        const button1 = document.getElementById("button1");
        const input1 = document.getElementById("input1");
        const button2 = document.getElementById("button2");
        const result1 = document.getElementById("result1");
        const result2 = document.getElementById("result2");

        button1.addEventListener("click", () => {
        const inputValue = input1.value;
        fetch(`/rust/${inputValue}`)
            .then(response => response.text())
            .then(data => {
            result1.textContent = data;
            });
        });

        button2.addEventListener("click", () => {
        fetch("/ts")
            .then(response => response.text())
            .then(data => {
            result2.textContent = data;
            });
        });
    </script>
    </body>
    </html>
    ```

## Deploy to cloud

!Note: The sqlite support is in preview in the Cloud. Please check here to get access: https://developer.fermyon.com/cloud/noops-sql-db#accessing-private-beta

The below will fail if you don't have access to the feature.

1. `spin deploy`

2. To run the db migration:

    1. run `spin cloud sqlite list` to get the db name

    2. run `sipn cloud sqlite execute <db_name> @db.sql`
