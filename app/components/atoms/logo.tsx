import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '~/lib/utils';

const iconVariants = cva('', {
    variants: {
        size: {
            '12': 'size-8',
            '36': 'size-28',
        },
    },

    defaultVariants: {
        size: '36',
    },
});

const logoLabelVariants = cva('', {
    variants: {
        size: {
            '12': 'text-2xl font-bold',
            '36': 'text-7xl font-bold',
        },
    },

    defaultVariants: {
        size: '36',
    },
});

const logoSubLabelVariants = cva('font-precious', {
    variants: {
        size: {
            '12': 'font-medium text-2xl',
            '36': 'text-7xl font-bold',
        },
        color: {
            red: 'text-red-500',
        },
    },
    defaultVariants: {
        size: '36',
        color: 'red',
    },
});

interface Props extends VariantProps<typeof logoLabelVariants> {}

const Logo = ({ size }: Props) => {
    return (
        <div className="flex items-center gap-2 h-12">
            <div className={cn(iconVariants({ size }))}>
                <img src="/icon.png" alt="DiceDB insight" className="block w-full dark:hidden" />
                <img src="/icon.png" alt="DiceDB insight" className="hidden w-full dark:block" />
            </div>
            <div className="flex items-center gap-2">
                <h1 className={logoLabelVariants({ size })}>DiceDB</h1>
                <h2 className={logoSubLabelVariants({ size })}>insight</h2>
            </div>
        </div>
    );
};

export default Logo;
