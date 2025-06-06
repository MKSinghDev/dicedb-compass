import { useParams } from 'react-router';

import Typography from '~/components/atoms/typography';

import type { Route } from './+types/$connection';

const ConnectionPage = () => {
    const params = useParams<Route.ComponentProps['params']>();
    return <Typography>{params.connection}</Typography>;
};

export default ConnectionPage;
