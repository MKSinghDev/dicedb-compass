import { Welcome } from '~/welcome/welcome';

export function meta() {
    return [{ title: 'DiceDB insight | Developed by MKSingh' }, { name: 'description', content: 'DiceDB insight | Developed by MKSingh' }];
}

export default function Home() {
    return <Welcome />;
}
