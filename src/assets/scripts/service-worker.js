

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

    if (event.request.method === 'POST' && url.pathname === '/pwa/share') {
        event.respondWith(
            (async () => {
                const formData = await event.request.formData();

                const link = formData.get('link');
                const isRedditLink = /^https?:\/\/(www\.)?reddit\.com\//.test(link);

                if (isRedditLink) {
                    const response = await caches.match(SHARE_PAGE);

                    if (response) {
                        const responseURL = new URL(response.url);
                        responseURL.searchParams.set('rlink', link);
                        response.url = responseURL;
                        return response;
                    } else {
                        return new Response(`Page: ${SHARE_PAGE} was not found in cache`, { status: 400 });
                    }

                } else {
                    return new Response('Not a Reddit link', { status: 400 });
                }
            }));
    } else {
        event.respondWith(fetch(event.request));
    }
});