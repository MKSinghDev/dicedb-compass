import { useNavigation } from 'react-router';

import RawLoader from './raw-loader';

const Loader = ({ label }: { label?: string }) => {
    const { state } = useNavigation();
    return state === 'submitting' ? (
        <div className="absolute inset-0 flex items-center justify-center backdrop-blur-sm">
            <div className="grid space-y-2 place-items-center">
                <RawLoader />
                {label || 'Please wait...'}
            </div>
        </div>
    ) : null;
};

export default Loader;
