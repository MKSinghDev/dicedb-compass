import { Loader } from 'lucide-react';

const RawLoader = ({ className }: { className?: string }) => (
    <div className="relative w-fit aspect-square">
        <Loader className={className} />
        <div className="absolute opacity-60 animate-spin rounded-full inset-0 bg-[conic-gradient(from_0deg,var(--color-muted)_0deg,var(--color-muted)_90deg,transparent_90deg,transparent_180deg,var(--color-muted)_180deg,var(--color-muted)_270deg,transparent_270deg,transparent_360deg)]" />
    </div>
);

export default RawLoader;
