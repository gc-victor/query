let lastModified = null;
const source = new EventSource(`${window.location.origin}/hot-reload?href=${window.location.href}`);
source.onmessage = ({ data: newLastModified }) => {
    if (lastModified && lastModified !== newLastModified) {
        window.location.reload();
    }
    lastModified = newLastModified;
};
