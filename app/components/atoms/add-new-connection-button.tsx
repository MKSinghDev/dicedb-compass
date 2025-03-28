import { PlusCircle } from 'lucide-react';

import { Button, type ButtonProps } from '~/components/ui/button';

interface Props extends ButtonProps {}

const AddNewConnectionButton = ({ children = 'Connect a DiceDB', ...props }: Props) => (
    <Button {...props}>
        <PlusCircle />
        {children}
    </Button>
);

export default AddNewConnectionButton;
