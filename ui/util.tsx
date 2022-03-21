import { useEffect,  useState } from "preact/hooks";

export function useEvent(target: EventTarget, event: string, handler: EventListenerOrEventListenerObject) {
    useEffect(() => {
        target.addEventListener(event, handler);
        return () => target.removeEventListener(event, handler);
    }, [])
}


export function useUrl(): [string, (newUrl: string) => string] {
    const [current, set] = useState(window.location.pathname);

    useEvent(window, 'popstate', () => set(window.location.pathname))

    function change(v: string): string {
        window.history.pushState({}, '', v)
        set(v)
        return v;
    }

    return [current, change]
}

export function classes(classes: {[key: string]: boolean}): string {
    return Object.keys(classes).filter((k) => classes[k]).join(" ")
}
