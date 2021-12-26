insert into best_shop_ever.address (id, country, city, street, zip)
values ('d576fafc-8743-40a0-9685-fa5ac9c690c5', 'Russia', 'Moscow', 'Tverskaya 12', 12345),
       ('fd385db0-da17-488e-a8ac-2b665295c034', 'Russia', 'Krasnoyarsk', 'Borisova 12', 31246),
       ('a8b6deab-0962-4fe8-a19e-b1811cc71aff', 'Hungary', 'Budapest', 'Dandar u.22', 1095);

insert into best_shop_ever.users (id, name, address_id)
values ('c8356372-432c-4f8e-9cfb-329d911817b1', 'Patia', 'd576fafc-8743-40a0-9685-fa5ac9c690c5'),
       ('314bb2dc-c400-460d-9d09-aaa5ba74dd21', 'Vasia', 'fd385db0-da17-488e-a8ac-2b665295c034'),
       ('f325dd43-e8cb-4aaf-9da2-b10ede8e42d0', 'Agnes', 'a8b6deab-0962-4fe8-a19e-b1811cc71aff');

insert into best_shop_ever.orders (id, user_id, address_id)
values ('6c54237f-7092-4d63-9227-449ffde3586c', 'c8356372-432c-4f8e-9cfb-329d911817b1', 'd576fafc-8743-40a0-9685-fa5ac9c690c5'),
       ('df911b93-826c-4bda-8719-90ae726ab0b8', 'c8356372-432c-4f8e-9cfb-329d911817b1', 'd576fafc-8743-40a0-9685-fa5ac9c690c5'),
       ('0da7e756-a01d-468c-b0e9-6c3bcc2ca72f', '314bb2dc-c400-460d-9d09-aaa5ba74dd21', 'fd385db0-da17-488e-a8ac-2b665295c034'),
       ('01da0436-e359-4609-99ff-2acf34dc1636', 'f325dd43-e8cb-4aaf-9da2-b10ede8e42d0', 'a8b6deab-0962-4fe8-a19e-b1811cc71aff'),
       ('0c25201f-c9a8-490f-8f5b-c22c799d337f', 'f325dd43-e8cb-4aaf-9da2-b10ede8e42d0', 'a8b6deab-0962-4fe8-a19e-b1811cc71aff');

insert into best_shop_ever.item (id, title, description, price)
values ('ae49f8c3-9b15-4ea8-9945-1ce1969e9884', 'For whom the bell tolls', 'book', '15'),
       ('4187d2e7-6240-4f8c-8191-93a58a363d50', 'Sweetest ever cake', 'dessert', '5'),
       ('68cdb740-323a-479b-aee2-948a5e9ec140', 'Coffee', 'drink', '2'),
       ('d360d8e2-329a-4815-b856-af2554c21594', 'Tesla Model 3', 'car', '2131213'),
       ('e2f80070-b6e6-4a97-92cd-9675d9fe165a', 'One Plus 8T', 'phone', '200'),
       ('ae3a5121-0bf3-40df-b867-dceb56f0bd3e', 'Coolest backpack', 'backpack', '20');

insert into best_shop_ever.order_item (order_id, item_id)
values ('6c54237f-7092-4d63-9227-449ffde3586c', 'ae3a5121-0bf3-40df-b867-dceb56f0bd3e'),
       ('6c54237f-7092-4d63-9227-449ffde3586c', '68cdb740-323a-479b-aee2-948a5e9ec140'),
       ('df911b93-826c-4bda-8719-90ae726ab0b8', '4187d2e7-6240-4f8c-8191-93a58a363d50'),
       ('0da7e756-a01d-468c-b0e9-6c3bcc2ca72f', 'd360d8e2-329a-4815-b856-af2554c21594'),
       ('01da0436-e359-4609-99ff-2acf34dc1636', 'ae49f8c3-9b15-4ea8-9945-1ce1969e9884'),
       ('01da0436-e359-4609-99ff-2acf34dc1636', '4187d2e7-6240-4f8c-8191-93a58a363d50'),
       ('0c25201f-c9a8-490f-8f5b-c22c799d337f', 'e2f80070-b6e6-4a97-92cd-9675d9fe165a'),
       ('0c25201f-c9a8-490f-8f5b-c22c799d337f', '68cdb740-323a-479b-aee2-948a5e9ec140'),
       ('0c25201f-c9a8-490f-8f5b-c22c799d337f', 'ae3a5121-0bf3-40df-b867-dceb56f0bd3e');