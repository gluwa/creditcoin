import { registerDealOrder } from './examples/register-deal-order'
import { registerAddress } from './examples/register-address';

const main = async () => {
    registerAddress();
    registerDealOrder();
}

main().catch(console.error)