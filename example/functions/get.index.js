export async function handleRequest(req) {
    try {
        return new Response("Index!", {
	        status: 200,
	        headers: {
	            "content-type": "text/html;charset=UTF-8",
	        },
	    });
	} catch (e) {
		return new Response("Error!", {
	        status: 500,
	        headers: {
	            "content-type": "text/html;charset=UTF-8",
	        },
	    });
	}
}