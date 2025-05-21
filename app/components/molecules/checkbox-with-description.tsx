import { useId } from 'react';

import type { CheckboxProps } from '@radix-ui/react-checkbox';
import { Checkbox } from '~/components/ui/checkbox';
import { Label } from '~/components/ui/label';

interface Props extends CheckboxProps {
    label: string;
    description: string;
}

const CheckboxWithDescription = ({ label, description, ...props }: Props) => {
    const id = useId();
    return (
        <div className="border-input has-data-[state=checked]:border-primary/50 relative flex w-full items-start gap-2 rounded-md border p-4 shadow-xs outline-none">
            <Checkbox id={id} className="order-1 after:absolute after:inset-0" aria-describedby={`${id}-description`} {...props} />
            <div className="grid grow gap-2">
                <Label htmlFor={id}>{label}</Label>
                <p id={`${id}-description`} className="text-muted-foreground text-xs">
                    {description}
                </p>
            </div>
        </div>
    );
};

export default CheckboxWithDescription;
