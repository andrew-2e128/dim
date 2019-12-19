import {
    FETCH_CARDS_START,
    FETCH_CARDS_OK,
    FETCH_CARDS_ERR,
    FETCH_CARD_START,
    FETCH_CARD_OK,
    FETCH_CARD_ERR
} from "./types.js";

export const fetchCards = (token, path) => async (dispatch) => {
    dispatch({ type: FETCH_CARDS_START });

    try {
        const config = {
            headers: {
                "authorization": token,
            }
        };
        const res = await fetch(path, config);

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_CARDS_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: FETCH_CARDS_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: FETCH_CARDS_ERR,
            payload: err
        });
    }
}

export const fetchCard = (token, id) => async (dispatch) => {
    dispatch({ type: FETCH_CARD_START });

    try {
        const config = {
            headers: {
                "authorization": token,
            }
        };
        const res = await fetch(`//${window.host}:8000/api/v1/media/${id}`, config);

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_CARD_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: FETCH_CARD_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: FETCH_CARD_ERR,
            payload: err
        });
    }
}
