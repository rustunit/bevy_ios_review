@import UIKit;
@import StoreKit;

void ios_request_review(UIView* view)
{
    [SKStoreReviewController requestReviewInScene:view.window.windowScene];
}
