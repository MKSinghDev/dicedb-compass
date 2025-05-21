import clsx from 'clsx';

import { type FieldMetadata, unstable_useControl as useControl } from '@conform-to/react';
import type { SelectProps } from '@radix-ui/react-select';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '~/components/ui/select';
import { cn } from '~/lib/utils';

interface FormSelectProps<T extends string> extends SelectProps {
    field: FieldMetadata<
        string,
        {
            [K in T]: string;
        },
        string[]
    >;
    label: string;
    description?: string;
    className?: string;
    options: Array<{ value: string; label: string }>;
    placeholder?: string;
}

const SelectWithLabel = <T extends string>({ field, label, description, className, options, placeholder, ...props }: FormSelectProps<T>) => {
    const control = useControl(field);
    return (
        <div
            className={cn(
                'relative w-full rounded-lg border border-input text-foreground bg-background shadow-sm shadow-black/5 transition-shadow focus-within:border-ring focus-within:outline-none focus-within:ring-[3px] focus-within:ring-ring/20 has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-50 [&:has(input:is(:disabled))_*]:pointer-events-none',
                clsx({
                    'border-destructive/80 text-destructive focus-within:border-destructive/80 focus-within:ring-destructive/20': field.errors,
                })
            )}
        >
            <label htmlFor={field.id} className="block px-3 pt-2 text-xs font-medium">
                {label}
            </label>
            <select name={field.name} defaultValue={field.initialValue ?? ''} className="sr-only" ref={control.register} aria-hidden tabIndex={-1} />
            <Select
                name={field.name}
                value={control.value}
                onValueChange={control.change}
                onOpenChange={open => {
                    if (!open) {
                        control.blur();
                    }
                }}
                {...props}
            >
                <SelectTrigger
                    name={field.name}
                    id={field.id}
                    aria-describedby={field.descriptionId}
                    aria-invalid={field.errors ? 'true' : 'false'}
                    className={cn('border-none bg-transparent shadow-none focus:ring-0 focus:ring-offset-0', className)}
                >
                    <SelectValue placeholder={placeholder ?? 'Select...'} />
                </SelectTrigger>
                <SelectContent>
                    {options.map(option => (
                        <SelectItem value={option.value} key={option.value}>
                            {option.label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
            {description ? <span className="text-xl">{description}</span> : null}
            {field.errors && <span className="text-xs text-destructive px-3 pb-2 block">{field.errors}</span>}
        </div>
    );
};

export default SelectWithLabel;
