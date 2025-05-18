import { Welcome } from '~/welcome/welcome';

export function meta() {
    return [{ title: 'DiceDB compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB compass | Developed by MKSingh' }];
}

export default function Home() {
    return <Welcome />;
}
