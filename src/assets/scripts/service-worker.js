

const CACHE_NAME = 'my-pwa-cache-v1';
const SHARE_PAGE = '/pwa/share_status.html';
const CACHED_URLS = [SHARE_PAGE];

self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => cache.addAll(CACHED_URLS))
    );
});

self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);

    console.log("Got request: ", url.pathname);
    if (event.request.method === 'POST' && url.pathname === '/pwa/share') {
        event.respondWith(
            (async () => {
                const formData = await event.request.formData();

                const file = formData.get('fff');
                if (!(file instanceof File) || file.type !== 'text/plain') {
                    return new Response('Not a plain text share', { status: 400 });
                }

                try {
                    const link = await readFileContent(file);
                    const isRedditLink = /^https?:\/\/(www\.)?reddit\.com\//.test(link);

                    if (isRedditLink) {

                        const newUrl = new URL(SHARE_PAGE, self.location.origin);
                        newUrl.searchParams.set('rlink', link);

                        return Response.redirect(newUrl.href, 303);
                    } else {
                        return new Response('No Reddit links found:', { status: 400 });
                    }
                } catch (error) {
                    return new Response(`Error reading file ${error}`, { status: 500 });
                }
            })());
    } else if (url.pathname === SHARE_PAGE) {
        (async () => {            
            const response = await caches.match(SHARE_PAGE);
            if (response) {
                return response;
            } else {
                event.respondWith(fetch(event.request));
            }
        })()
    } else {
        event.respondWith(fetch(event.request));
    }
});

function readFileContent(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();

        // Resolve the promise when the reading is done
        reader.onload = () => {
            resolve(reader.result);
        };

        // Reject the promise on error
        reader.onerror = () => {
            reject(reader.error);
        };

        // Start reading the file as text
        reader.readAsText(file);
    });
}