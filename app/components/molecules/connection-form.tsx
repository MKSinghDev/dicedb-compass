import { Form, useActionData } from 'react-router';

import { useForm } from '@conform-to/react';
import { parseWithZod } from '@conform-to/zod/v4';
import InputWithLabel from '~/components/atoms/form-elements/input-with-label';
import SubmitButton from '~/components/atoms/form-elements/submit-button';
import CheckboxWithDescription from '~/components/molecules/checkbox-with-description';
import { Button } from '~/components/ui/button';
import { DialogClose } from '~/components/ui/dialog';
import { Separator } from '~/components/ui/separator';
import { schema } from '~/lib/schema/connection';

const ConnectionForm = () => {
    const lastResult = useActionData();
    const [form, fields] = useForm({
        lastResult,
        shouldValidate: 'onSubmit',
        shouldRevalidate: 'onInput',
        onValidate({ formData }) {
            return parseWithZod(formData, { schema });
        },
    });

    return (
        <Form className="flex flex-col space-y-5" method="POST" id={form.id} onSubmit={form.onSubmit}>
            <div className="flex flex-col gap-4">
                <div className="flex gap-3">
                    <InputWithLabel field={fields.host} label="Host" placeholder="localhost" />
                    <InputWithLabel field={fields.port} label="Port" placeholder="7379" />
                </div>
                <Separator />
                <InputWithLabel field={fields.name} label="Name" placeholder="Production DB" />
            </div>
            <CheckboxWithDescription name="isDefault" label="Set as default database" description="Default databese will be loaded on app startup" />
            <div className="flex items-center gap-3 justify-between w-full mt-4">
                <DialogClose asChild>
                    <Button type="button" variant="secondary">
                        Cancel
                    </Button>
                </DialogClose>
                <div className="flex flex-row-reverse gap-3">
                    <SubmitButton name="action" value="save-and-connect">
                        Save & Connect
                    </SubmitButton>
                    <Button variant="outline" name="action" value="connect">
                        Connect
                    </Button>
                    <Button variant="outline" name="action" value="save">
                        Save
                    </Button>
                </div>
            </div>
        </Form>
    );
};

export default ConnectionForm;
