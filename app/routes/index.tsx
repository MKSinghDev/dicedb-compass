import { Welcome } from '~/client/welcome/welcome';

export function meta() {
    return [{ title: 'DiceDB Insight | Developed by MKSingh' }, { name: 'description', content: 'DiceDB Insight | Developed by MKSingh' }];
}

export default function Home() {
    return <Welcome />;
}
