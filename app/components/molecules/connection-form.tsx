import { useState } from 'react';
import { Form, useActionData, useNavigation } from 'react-router';
import type { Route } from '+/_layout+/connection+/+types/add';
import { CircleCheckBig } from 'lucide-react';

import { useForm } from '@conform-to/react';
import { parseWithZod } from '@conform-to/zod/v4';
import InputWithLabel from '~/components/atoms/form-elements/input-with-label';
import SubmitButton from '~/components/atoms/form-elements/submit-button';
import CheckboxWithDescription from '~/components/molecules/checkbox-with-description';
import { Button } from '~/components/ui/button';
import { DialogClose } from '~/components/ui/dialog';
import { Separator } from '~/components/ui/separator';
import { schema } from '~/lib/schema/connection';

import RawLoader from '../atoms/raw-loader';
import type { Action } from '../pages/home/interface';

const ConnectionForm = () => {
    const [actionName, setActionName] = useState<Action>();
    const [form, fields] = useForm({
        shouldValidate: 'onSubmit',
        shouldRevalidate: 'onInput',
        onValidate({ formData }) {
            return parseWithZod(formData, { schema });
        },
    });
    const lastResult = useActionData<Route.ComponentProps['actionData']>();
    const { state } = useNavigation();
    return (
        <Form
            className="flex flex-col space-y-5"
            method="POST"
            id={form.id}
            onSubmit={e => {
                form.onSubmit(e);
                const submitter = (e.nativeEvent as SubmitEvent).submitter as HTMLButtonElement;
                setActionName(submitter.value as Action);
            }}
        >
            <div className="flex flex-col gap-4">
                <div className="flex gap-3">
                    <InputWithLabel field={fields.host} label="Host" placeholder="localhost" disabled={state === 'submitting'} />
                    <InputWithLabel field={fields.port} label="Port" placeholder="7379" disabled={state === 'submitting'} />
                </div>
                <Separator />
                <InputWithLabel field={fields.name} label="Name" placeholder="Production DB" disabled={state === 'submitting'} />
            </div>
            <CheckboxWithDescription
                name="isDefault"
                label="Set as default database"
                description="Default databese will be loaded on app startup"
                disabled={state === 'submitting'}
            />
            <div className="flex items-center gap-3 justify-between w-full mt-4">
                <DialogClose asChild>
                    <Button type="button" variant="secondary" disabled={state === 'submitting'}>
                        Cancel
                    </Button>
                </DialogClose>
                <div className="flex flex-row-reverse gap-3">
                    <SubmitButton name="action" value="save-and-connect" disabled={state === 'submitting'}>
                        {state === 'submitting' && actionName === 'save-and-connect' ? (
                            <>
                                Processing <RawLoader />
                            </>
                        ) : lastResult?.type === 'success' && lastResult.data?.action === 'save-and-connect' && actionName === 'save-and-connect' ? (
                            <>
                                Done <CircleCheckBig className="text-emerald-400 dark:text-emerald-500" />
                            </>
                        ) : (
                            <>Save &amp; Connect</>
                        )}
                    </SubmitButton>
                    <Button variant="outline" name="action" value="connect" disabled={state === 'submitting'}>
                        {state === 'submitting' && actionName === 'connect' ? (
                            <>
                                Connecting <RawLoader />
                            </>
                        ) : lastResult?.type === 'success' && lastResult.data?.action === 'connect' && actionName === 'connect' ? (
                            <>
                                Connected <CircleCheckBig className="text-emerald-500 dark:text-emerald-400" />
                            </>
                        ) : (
                            'Connect'
                        )}
                    </Button>
                    <Button variant="outline" name="action" value="save" disabled={state === 'submitting'}>
                        {state === 'submitting' && actionName === 'save' ? (
                            <>
                                Saving <RawLoader />
                            </>
                        ) : lastResult?.type === 'success' && lastResult.data?.action === 'save' && actionName === 'save' ? (
                            <>
                                Saved <CircleCheckBig className="text-emerald-500 dark:text-emerald-400" />
                            </>
                        ) : (
                            'Save'
                        )}
                    </Button>
                </div>
            </div>
        </Form>
    );
};

export default ConnectionForm;
