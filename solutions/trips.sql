# Write your MySQL query statement below

let start:string = '2013-10-01';
let end:string = '2013-10-01';
select
  request_at,
  round(
    (cast ( numerator as long )
    / cast( denominator as long )
    ), 
    2) as cancellation_rate
from
(
    select
    request_at
    sum(case (
        if category == 'cancelled_by_client' then 1
        else if category == 'cancelled_by_driver' then 1
        else 0
    ) as numerator
    count(*) as denominator
    from trips
    where
    request_at between(@start, @end)
    anti join on user_id (
        select
            user_id from users
        where
            banned = 'No'
            & (role = 'Driver' || role = 'Parter')
    )
)

