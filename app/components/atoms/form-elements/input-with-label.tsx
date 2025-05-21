import clsx from 'clsx';
import type { ComponentProps } from 'react';

import type { FieldMetadata } from '@conform-to/react';
import { cn } from '~/lib/utils';

interface FormInputProps<T extends string> extends ComponentProps<'input'> {
    field: FieldMetadata<
        string | number,
        {
            [K in T]?: string | number;
        },
        string[]
    >;
    label: string;
    description?: string;
}

const InputWithLabel = <T extends string>({ field, label, description, className, ...props }: FormInputProps<T>) => {
    return (
        <label
            // htmlFor={field.id}
            className={cn(
                'relative w-full rounded-lg border border-input text-foreground bg-background shadow-sm shadow-black/5 transition-shadow focus-within:border-ring focus-within:outline-none focus-within:ring-[3px] focus-within:ring-ring/20 has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-50 [&:has(input:is(:disabled))_*]:pointer-events-none',
                clsx({
                    'border-destructive/80 text-destructive focus-within:border-destructive/80 focus-within:ring-destructive/20': field.errors,
                })
            )}
        >
            <span className="block px-3 pt-2 text-xs font-medium">{label}</span>
            <input
                name={field.name}
                key={field.key}
                className={cn(
                    'flex h-10 w-full bg-transparent px-3 pb-2 text-sm text-foreground placeholder:text-muted-foreground/70 focus-visible:outline-none invalid:border-destructive/80 invalid:text-destructive invalid:focus-visible:border-destructive/80 invalid:focus-visible:ring-destructive/20',
                    className
                )}
                id={field.id}
                defaultValue={field.initialValue}
                aria-describedby={field.descriptionId}
                aria-invalid={field.errors ? 'true' : 'false'}
                {...props}
            />
            {description ? <span className="text-xl">{description}</span> : null}
            {field.errors && <span className="text-xs text-destructive px-3 pb-2 block">{field.errors}</span>}
        </label>
    );
};

export default InputWithLabel;
