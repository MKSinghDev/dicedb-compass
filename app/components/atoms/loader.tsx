import { useNavigation } from 'react-router';
import { LoaderIcon } from 'lucide-react';

const Loader = () => {
    const { state } = useNavigation();
    return state === 'submitting' ? (
        <div className="absolute inset-0 flex items-center justify-center backdrop-blur-sm">
            <div className="grid space-y-2 place-items-center">
                <div className="relative w-fit aspect-square">
                    <LoaderIcon />
                    <div className="absolute opacity-60 animate-spin rounded-full inset-0 bg-[conic-gradient(from_0deg,var(--color-muted)_0deg,var(--color-muted)_90deg,transparent_90deg,transparent_180deg,var(--color-muted)_180deg,var(--color-muted)_270deg,transparent_270deg,transparent_360deg)]" />
                </div>
                Please wait...
            </div>
        </div>
    ) : null;
};

export default Loader;
