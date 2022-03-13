import { useState, useEffect } from "preact/hooks";

export type Res<T> =
    | { status: 'loading' }
    | { status: 'ok', data: T }
    | { status: 'err', msg: string }
    ;

export function useReq<ReqT, ResT>(method: 'get' | 'post', path: string, req?: ReqT): Res<ResT> {
    const [res, setRes] = useState<Res<ResT>>({status: 'loading'});

    const body = req && JSON.stringify(req);

    useEffect(() => {
        const opts = {
            method: method,
            headers: {
                ... req && {'content-type' : 'application/json'}
            },
            body,
        };
        fetch(path, opts)
            .then(async (response) => {
                if (response.ok) {
                    setRes({ status: 'ok', data: await response.json() })
                } else {
                    setRes({ status: 'err', msg: (await response.json()).message })
                }
            })
            .catch(error => setRes({ status: 'err', msg: error }));
    }, [method, path, body]);

    return res;
}