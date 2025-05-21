import { useNavigation } from 'react-router';

import { Button, type ButtonProps } from '~/components/ui/button';

const SubmitButton = ({ children, disabled, ...props }: ButtonProps) => {
    const { state } = useNavigation();
    return (
        <Button type="submit" disabled={disabled || state === 'submitting'} {...props}>
            {children}
        </Button>
    );
};

export default SubmitButton;
