import { z } from 'zod/v4';

export const schema = z.object({
    host: z
        .string({ message: 'Host is required' })
        .min(5, { message: 'Host must be at least 5 characters long' })
        .refine(
            value => {
                // Basic domain validation (no protocol, no paths, no ports)
                return /^([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$/i.test(value) || /^[a-z0-9]+(-[a-z0-9]+)*$/i.test(value);
            },
            { message: 'Must be a valid domain (e.g., example.com)' }
        ),

    port: z
        .number({ message: 'Provide a port number' })
        .int({ message: 'Port must be an integer' })
        .min(80, { message: 'Port must be at least 1' })
        .max(65535, { message: 'Port cannot exceed 65535' }),

    name: z
        .string({ message: 'Name is required' })
        .min(2, { message: 'Name must be at least 2 characters long' })
        .max(155, { message: 'Name cannot exceed 155 characters' })
        .trim()
        .refine(val => val.length > 0, { message: 'Name cannot be empty' }),

    isDefault: z
        .enum(['on'], {
            message: 'Must be either "on" or "undefined"',
        })
        .optional()
        .transform(value => value === 'on'),
});
