import { z } from 'zod/v4';

export const schema = z.object({
    name: z.string({ message: 'Key cannot be empty' }),
    value: z.string().optional(),
});
