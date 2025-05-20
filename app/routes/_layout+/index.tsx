import { loadStronghold } from '~/lib/stronghold';
import { Welcome } from '~/welcome/welcome';

export function meta() {
    return [{ title: 'DiceDB compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB compass | Developed by MKSingh' }];
}

export const clientLoader = async () => {
    return await loadStronghold({ clientNmae: 'dicedb-compass' });
};

export default function Home() {
    return <Welcome />;
}
