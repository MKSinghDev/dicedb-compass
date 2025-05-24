import { useEffect, useRef } from 'react';
import { useActionData, useSearchParams } from 'react-router';

import { getJsonData } from '~/lib/utils';
import { dispatchToast, type MessageResponse, renderMessage } from '~/lib/utils/message-handler';

const MessageToaster = ({ searchParamName = 'message' }: { searchParamName?: string }) => {
    const [searchParams, setSearchParams] = useSearchParams();
    const hasDispatchedRef = useRef(false);

    useEffect(() => {
        if (hasDispatchedRef.current) {
            hasDispatchedRef.current = false;
            return;
        }

        const message = getJsonData(searchParams.get(searchParamName));
        if (message) {
            dispatchToast(message);
            setSearchParams(prev => {
                prev.delete(searchParamName);
                return prev;
            });
            hasDispatchedRef.current = true;
        }
    }, [searchParams, searchParamName, setSearchParams]);
    return null;
};

const ActionMessage = () => {
    const data = useActionData<MessageResponse>();
    return data && renderMessage(data);
};

const ActionMessageToaster = () => {
    const message = useActionData<MessageResponse>();
    const hasDispatchedRef = useRef(false);

    useEffect(() => {
        if (hasDispatchedRef.current) return;
        if (message) {
            dispatchToast(message);
            hasDispatchedRef.current = true;
        }
    }, [message]);
    return null;
};

export { ActionMessage, ActionMessageToaster, MessageToaster };
